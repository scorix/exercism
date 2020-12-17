-module(accumulate).

-export([accumulate/2]).

accumulate(Fn, Ls) ->
  tail_accumulate(Fn, [], Ls).

tail_accumulate(_, HeadList, []) ->
  HeadList;
tail_accumulate(Fn, HeadList, [Head|Tail]) ->
  tail_accumulate(Fn, HeadList++[Fn(Head)], Tail).