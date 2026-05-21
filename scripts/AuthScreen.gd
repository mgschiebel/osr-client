extends Control

@onready var username_edit = $CenterContainer/VBoxContainer/UsernameEdit
@onready var password_edit = $CenterContainer/VBoxContainer/PasswordEdit
@onready var enter_button = $CenterContainer/VBoxContainer/EnterButton
@onready var error_label = $CenterContainer/VBoxContainer/ErrorLabel

func _ready():
    # Connect to GameState signals
    GameState.connect("auth_succeeded", Callable(self, "_on_auth_succeeded"))
    GameState.connect("auth_failed", Callable(self, "_on_auth_failed"))

func _on_enter_pressed():
    var username = username_edit.text
    var password = password_edit.text

    if username.is_empty() or password.is_empty():
        error_label.text = "Please enter username and password"
        return

    error_label.text = "Authenticating..."
    enter_button.disabled = true

    GameState.authenticate(username, password)

func _on_auth_succeeded(token: String, game_server: String):
    error_label.text = ""
    enter_button.disabled = false
    GameState.transition_to_character_select()

func _on_auth_failed(error: String):
    error_label.text = "Auth failed: " + error
    password_edit.text = ""
    enter_button.disabled = false
