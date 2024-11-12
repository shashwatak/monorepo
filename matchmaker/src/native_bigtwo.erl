-module(native_bigtwo).
-export([perform/0]).
-nifs([perform/0]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("priv/libnative_bigtwo", 0).

perform() ->
    exit(nif_library_not_loaded).

