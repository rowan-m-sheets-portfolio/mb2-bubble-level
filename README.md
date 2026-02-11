# Bubble level program for the micro:bit v2
Code for the BBC micro:bit v2 that uses its IMU to act as a bubble level.

## Authors
main.rs: Rowan Sheets

## How to run
1. Make sure you have all the requirements to use embedded rust with the BBC micro:bit (instructions available
[here](https://docs.rust-embedded.org/discovery-mb2/index.html))
2. Run the command `cargo embed`
3. The B button on the microbit can be used to change the level to "fine" mode which is more precise. The A button can then be used to put it back into course mode.

## Process
To start with I researched how to interact with the micro:bit's IMU using the i2c protocal. I then used the data recived from the IMU to map to the display by clamping the x and y value to -500 and 500 mG and then using a match statment to break it up in increments of 200 mG. Once I had this working I implemented the "fine" mode by adding a control variable that if true would instead clamp the x and y value to -50 and 50 and break it up by increments of 20 mG. Finally I added code to read the state of the buttons and set the value of the control variable accordingly.