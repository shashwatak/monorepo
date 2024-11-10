import app/router
import gleeunit
import gleeunit/should
import wisp/testing

pub fn main() {
  gleeunit.main()
}

// copied from wisp examples
// 01-routing
pub fn get_home_page_test() {
  let request = testing.get("/", [])
  let response = router.handle_request(request)
  response.status
  |> should.equal(200)
}

pub fn post_home_page_test() {
  let request = testing.post("/", [], "a body")
  let response = router.handle_request(request)
  response.status
  |> should.equal(405)
}

pub fn page_not_found_test() {
  let request = testing.get("/nothing-here", [])
  let response = router.handle_request(request)
  response.status
  |> should.equal(404)
}
// 02-working-with-form-data
