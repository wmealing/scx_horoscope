# VHS tape file for scx_horoscope demo
# Run with: vhs demo.tape

# Output settings
Output demo.gif
Set Shell "bash"
Set FontSize 16
Set Width 1600
Set Height 1000
Set Padding 25
Set Theme "Catppuccin Mocha"

# Typing speed
Set TypingSpeed 20ms

# Start recording
Type "# scx_horoscope - Astrological CPU Scheduler" Enter
Type "# Let the cosmos guide our CPU scheduling!" Enter
Sleep 500ms

Type "sudo cargo run -- --cosmic-weather --debug-decisions"
Enter

# Wait for scheduler to start and show some decisions
Sleep 1s

# Stop the scheduler with Ctrl+C
Ctrl+C

Sleep 10s
