import app/router
import gleam/erlang/process
import gleam/int
import gleam/io
import mist
import wisp
import wisp/wisp_mist

@external(erlang, "native_bigtwo", "truly_random")
pub fn truly_random() -> int

pub fn main() {
  let a = truly_random()
  io.print(int.to_string(a))
  io.print(int.to_string(a))
  io.print(int.to_string(a))
  io.print(int.to_string(a))
  wisp.configure_logger()
  let secret_key_base = wisp.random_string(64)
  let assert Ok(_) =
    wisp_mist.handler(router.handle_request, secret_key_base)
    |> mist.new
    |> mist.port(8000)
    |> mist.start_http
  process.sleep_forever()
}
