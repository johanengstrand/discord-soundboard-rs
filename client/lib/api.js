/// @brief Sends a request to an endpoint of the API
/// @throws Error if the request fails or the server has as an error
/// @returns Object the JSON data returned by the server
async function sendRequest(endpoint, method, data = null) {
  const url = `/api/${endpoint}`;
  const options = { method };
  if (data) {
    options['body'] = JSON.stringify(data);
  }

  const res = await fetch(url, options);

  console.debug(res);

  if (res.ok) {
    const json = await res.json();
    console.debug(json);

    if (json.success) {
      return json.data;
    } else {
      throw new Error(json.error);
    }
  } else {
    throw new Error(`Failed to send request to ${url}`);
  }
}

async function get(endpoint) {
  return sendRequest(endpoint, 'GET', null);
}

async function post(endpoint, data) {
  return sendRequest(endpoint, 'POST', data);
}

/// @brief Fetches all available tracks from the server
/// @throws Error if the tracks could not be fetched
/// @returns Object containing the path of all available tracks
export async function fetchTracks() {
  return await get('tracks');
}

/// @brief Fetches all available tracks from the server
/// @throws Error if the tracks could not be fetched
/// @returns Object containing the path of all available tracks
export async function playTrack(path) {
  return await post('play', path);
}
