addEventListener("fetch", (event) => {
  event.respondWith(handleEvent(event));
});

/**
 * Fetch and log a event
 * @param {Event} event
 */
async function handleEvent(event) {
  const { handle_event } = wasm_bindgen;
  await wasm_bindgen(wasm);
  const response = handle_event(event, event.request);
  return response;
  // return new Response(response, { status: 200 });
}
