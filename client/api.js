/// @brief Sends a request to an endpoint of the API
/// @throws Error if the request fails or the server has as an error
/// @returns Object the JSON data returned by the server
async function sendRequest(endpoint, method, data = null) {
  const url = `/api/${endpoint}`;
  const options = {
    method,
    headers: {
      'Content-Type': 'application/json;charset=utf-8'
    },
  };

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

/// @brief Attempts playback of a track based on the file path
/// @throws Error if the track could not be played
/// @returns Object containing metadata about the track
export async function playTrack(path) {
  return await post('play', path);
}

/// @brief Stops the playback of a track
/// @throws Error if the current track could not be stopped
/// @returns nothing
export async function stopTrack() {
  return await post('stop', {});
}

/// @brief Favorites a track and saves it on the server
/// @throws Error if the track could not be added to the favorite category
/// @returns nothing
export async function favoriteTrack(track) {
  return await post('favorite', track);
}

/// @brief Unfavorites a track and saves it on the server
/// @throws Error if the track could not be removed from the favorite category
/// @returns nothing
export async function unfavoriteTrack(track) {
  return await post('unfavorite', track);
}
