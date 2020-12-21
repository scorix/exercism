-module(raindrops).

-export([convert/1]).


convert(Number) -> convert_and_concat(Number, 0, []).

convert_and_concat(Number, Factor, Sound) when Number rem 3 == 0, Factor =/= 3 -> convert_and_concat(Number div 3, 3, lists:concat([Sound, "Pling"]));
convert_and_concat(Number, Factor, Sound) when Number rem 5 == 0, Factor =/= 5 -> convert_and_concat(Number div 5, 5, lists:concat([Sound, "Plang"]));
convert_and_concat(Number, Factor, Sound) when Number rem 7 == 0, Factor =/= 7 -> convert_and_concat(Number div 7, 7, lists:concat([Sound, "Plong"]));
convert_and_concat(Number, _, []) -> lists:concat([Number]);
convert_and_concat(_, _, Sound) -> Sound.