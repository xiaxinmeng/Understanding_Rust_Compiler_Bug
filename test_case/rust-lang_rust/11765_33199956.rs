 sml
(* Listing 3.5: Directional channels signature *)

signature DIR_CHAN =
    sig
        type ’a in_chan and ’a out_chan

        val channel : unit -> (’a in_chan * ’a out_chan)
        val recv    : ’a in_chan -> ’a
        val send    : (’a out_chan * ’a) -> unit

        val recvEvt : ’a in_chan -> ’a event
        val sendEvt : (’a out_chan * ’a) -> unit event
    end;
