-module(posix_queue).

%% API
-export([send/2]).

%% NIF
-export([load/0]).
-on_load(load/0).

%%=============================================================================
%% NIF API

load() ->
    SoName = filename:join(priv(), atom_to_list(?MODULE)),
    ok = erlang:load_nif(SoName, 0).

not_loaded(Line) ->
    erlang:nif_error({error, {not_loaded, [{module, ?MODULE}, {line, Line}]}}).

%%=============================================================================
%% Internal Functions

priv()->
  case code:priv_dir(posix_queue) of
      {error, _} ->
          EbinDir = filename:dirname(code:which(?MODULE)),
          AppPath = filename:dirname(EbinDir),
          filename:join(AppPath, "priv");
      Path ->
          Path
  end.

send(_Name, _Msg) ->
    not_loaded(?LINE).
