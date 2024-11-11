-module(native_bigtwo).
-export([truly_random/0]).
-nifs([truly_random/0]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("priv/libnative_bigtwo", 0).

truly_random() ->
    exit(nif_library_not_loaded).

