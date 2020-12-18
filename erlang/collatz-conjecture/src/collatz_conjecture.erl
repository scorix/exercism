-module(collatz_conjecture).

-export([steps/1]).


steps(N) ->
    count_steps(0, N).

count_steps(_, N) when N =< 0 ->
    erlang:error(badarg);
count_steps(S, 1) ->
    S;
count_steps(S, N) when N rem 2 == 0 ->
    count_steps(S+1, N div 2);
count_steps(S, N) ->
    count_steps(S+1, N*3+1).