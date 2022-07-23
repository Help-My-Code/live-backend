# Live Backend

Live backend is a service for handling Websocket communication with PimpMyCode live editor.

## Main feature

- Share CodeUpdate with other user
- Share compilation result with other user also

## Tools used

- Actix and Actix Web, tokio for async handling of exchange based on worker.
- Redis for save state of the room
