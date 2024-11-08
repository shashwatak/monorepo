import app/web
import gleam/string_builder
import wisp.{type Request, type Response}

pub fn handle_request(req: Request) -> Response {
  use _req <- web.middleware(req)

  case wisp.path_segments(req) {
    [] -> wisp.ok()
    ["greet", name] -> greet_handler(name)
    _ -> wisp.not_found()
  }
}

fn greet_handler(name: String) -> Response {
  case name {
    "shashwat" -> {
      string_builder.from_string("Hello, Boss!")
      |> wisp.json_response(200)
    }
    _ -> {
      string_builder.from_string("Hello, " <> name <> "!")
      |> wisp.json_response(200)
    }
  }
}
