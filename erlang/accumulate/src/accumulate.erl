-module(accumulate).

-export([accumulate/2]).

accumulate(_, []) -> [];
accumulate(Fn, [Head|Tail]) ->
  [Fn(Head) | accumulate(Fn, Tail)].
