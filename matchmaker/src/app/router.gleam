import app/web
import gleam/http.{Get}
import gleam/string_builder
import wisp.{type Request, type Response}

pub fn handle_request(req: Request) -> Response {
  use _req <- web.middleware(req)

  case wisp.path_segments(req) {
    [] -> handler(req)
    _ -> wisp.not_found()
  }
}

fn handler(req: Request) -> Response {
  use <- wisp.require_method(req, Get)
  wisp.ok()
}
