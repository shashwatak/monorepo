import app/web
import gleam/http.{Get, Post}
import gleam/list
import gleam/result
import gleam/string_builder
import wisp.{type Request, type Response}

@external(erlang, "native_bigtwo", "perform")
pub fn perform() -> void

// 02-working-with-form-data

pub fn handle_request(req: Request) -> Response {
  use req <- web.middleware(req)

  // For GET requests, show the form,
  // for POST requests we use the data from the form
  case req.method {
    Get -> show_form()
    Post -> handle_form_submission(req)
    _ -> wisp.method_not_allowed(allowed: [Get, Post])
  }
}

pub fn show_form() -> Response {
  // In a larger application a template library or HTML form library might
  // be used here instead of a string literal.
  perform()
  let html =
    string_builder.from_string(
      "<form method='post'>
        <label>play:
          <input type='text' name='play'>
        </label>
        <input type='submit' value='Submit'>
      </form>",
    )
  wisp.ok()
  |> wisp.html_body(html)
}

pub fn handle_form_submission(req: Request) -> Response {
  perform()
  use formdata <- wisp.require_form(req)

  let result = {
    use play <- result.try(list.key_find(formdata.values, "play"))
    let greeting = "you played " <> wisp.escape_html(play) <> "!"
    Ok(greeting)
  }

  // An appropriate response is returned depending on whether the form data
  // could be successfully handled or not.
  case result {
    Ok(content) -> {
      let html = string_builder.from_string("<form method='post'>
        <label>" <> content <> "
          <input type='text' name='play'>
        </label>
        <input type='submit' value='Submit'>
      </form>")
      wisp.ok()
      |> wisp.html_body(html)
    }
    Error(_) -> {
      wisp.bad_request()
    }
  }
}
