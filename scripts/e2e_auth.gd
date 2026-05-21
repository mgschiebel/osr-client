extends SceneTree

var auth_succeeded = false
var auth_failed = false

func _ready():
    # Connect to GameState signals
    GameState.connect("auth_succeeded", Callable(self, "_on_auth_succeeded"))
    GameState.connect("auth_failed", Callable(self, "_on_auth_failed"))

    # Start mock server (assumes it's already running)
    # Call authenticate
    var username = OS.get_environment_variable("OSR_TEST_USER", "testuser")
    var password = OS.get_environment_variable("OSR_TEST_PASS", "testpass")

    GameState.authenticate(username, password)

    # Wait for signal (timeout after 10 seconds)
    await get_tree().create_timer(10.0).timeout

    if auth_succeeded:
        print("E2E Auth Test: PASSED")
        get_tree().quit(0)
    else:
        print("E2E Auth Test: FAILED")
        get_tree().quit(1)

func _on_auth_succeeded(token: String, game_server: String):
    print("Auth succeeded! Token: %s, Game Server: %s" % [token, game_server])
    auth_succeeded = true

func _on_auth_failed(error: String):
    print("Auth failed: " + error)
    auth_failed = true
