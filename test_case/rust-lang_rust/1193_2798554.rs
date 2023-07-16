 erlang
-module(t).
-export([f/1]).

f(V) ->
   X = 1,
   case V of
      X -> true;
      0 -> false;
      Z -> new_binding
    end.
