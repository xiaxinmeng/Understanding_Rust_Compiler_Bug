 erlang
start () ->
  %% Get the Pid of the new task
  Pid = spawn (fun () -> some_func ([]) end),
  %% Send a message to the Pid task 
  %% {_} is a tuple and "add" like an enum item
  Pid ! {add, "hello"},
  Pid ! {show, self ()}, %% self () is the Pid of the current task
  receive %% Receive from the current port
    {data, List} ->
      io:format ("Liste ~w ~n", [List])
  end.

some_func (List) ->
  receive %% Receive Messages
    {add, Data} -> %% Pattern matching
       some_func ([Data | List]);
    {show, Pid} ->
      Pid ! {data, list},
      some_func (List);
    quit ->
      ok
   end.
