-module(genh3).
-export([to_serialized_h3/3]).

-define(TimeIt(Src, Name, Fun),
    time_it(Src, Name, fun() -> Fun end)
).

to_serialized_h3(GeoJSONSrc, BinFilePath, H3Resolution) ->
    try
        {ok, JSONb} = file:read_file(GeoJSONSrc),
        JSON = jsx:decode(JSONb, []),
        Polyfill = ?TimeIt(GeoJSONSrc, polyfill, to_polyfills(JSON, H3Resolution)),
        Flattened = ?TimeIt(GeoJSONSrc, flatten, lists:flatten(Polyfill)),
        Deduped = ?TimeIt(GeoJSONSrc, dedup, lists:usort(Flattened)),
        Compacted = ?TimeIt(GeoJSONSrc, compact, h3:compact(Deduped)),
        Sorted = ?TimeIt(
            GeoJSONSrc,
            sort,
            lists:sort(
                fun(A, B) -> h3:get_resolution(A) < h3:get_resolution(B) end,
                Compacted
            )
        ),
        LEU64Bin = ?TimeIt(GeoJSONSrc, serialize, <<
            <<H3:64/integer-unsigned-little>>
         || H3 <- Sorted
        >>),
        file:write_file(BinFilePath, LEU64Bin)
    catch
        Exception ->
            io:format(standard_error, "~p", [Exception]),
            erlang:halt(1)
    end.

time_it(GeoJSONSrc, StepName, Fun) ->
    {Duration, Val} = timer:tc(Fun),
    case Duration of
        D when D < 1.0e+3 ->
            io:format("~s: ~s took ~p us\n", [GeoJSONSrc, StepName, D]);
        D when D < 1.0e+6 ->
            io:format("~s: ~s took ~p ms\n", [GeoJSONSrc, StepName, D / 1.0e+3]);
        D ->
            io:format("~s: ~s took ~p s\n", [GeoJSONSrc, StepName, D / 1.0e+6])
    end,
    Val.

%%
%% Example:
%%
%% ```
%% %% First download a map from https://geojson-maps.ash.ms
%% {ok, JSONb} = file:read_file("custom.geo.json"),
%% JSON = jsx:decode(JSONb, []),
%% Poly = h3:to_polyfills(JSON, 8),
%% '''
-spec to_polyfills(JSON :: map(), Resolution :: h3:resolution()) -> [[h3:h3index(), ...], ...].
to_polyfills(#{<<"features">> := Features}, Resolution) ->
    lists:map(
        fun(Feature) ->
            to_polyfills(Feature, Resolution)
        end,
        Features
    );
to_polyfills(#{<<"geometry">> := Geometry}, Resolution) ->
    to_polyfills(Geometry, Resolution);
to_polyfills(
    #{<<"type">> := <<"MultiPolygon">>, <<"coordinates">> := Coordinates},
    Resolution
) ->
    geojson_parse_polygons(Coordinates, Resolution);
to_polyfills(
    #{<<"type">> := <<"Polygon">>, <<"coordinates">> := Coordinates},
    Resolution
) ->
    h3:polyfill(geojson_parse_polygon(Coordinates), Resolution).

geojson_parse_polygons(Polygons, Resolution) ->
    lists:map(
        fun(P) -> h3:polyfill(geojson_parse_polygon(P), Resolution) end,
        Polygons
    ).

geojson_parse_polygon(OutlineAndHoles) ->
    lists:map(fun(OH) -> geojson_transform_coordinates(OH) end, OutlineAndHoles).

geojson_transform_coordinates(CoordinateList) ->
    lists:map(fun([Lat, Lon]) -> {float(Lon), float(Lat)} end, CoordinateList).
