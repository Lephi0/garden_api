# Garden Api
Application to control Conbee stick via deCONZ REST-API. The goal of this project is to get to know Rust and the ability to control the LED light for the plants using a Raspberry Pi.

## Hardware needed
Since Conbee 2 is a universal zigbee controller only the Conbee 2 is a must have. Every other device is replaceable.
- [Conbee 2](https://phoscon.de/en/conbee2/software)
- [Ikea Traddfri Control Outlet](https://www.ikea.com/us/en/p/tradfri-wireless-control-outlet-30356169/)
- [Xiaomi Mi Light Detection Sensor](https://xiaomi-mi.com/sockets-and-sensors/xiaomi-mi-light-detection-sensor-zigbee-30/) (Not sure if 3.0 version)
- [Aquara Temerature and Humidity Sensor](https://www.aqara.com/us/temperature_humidity_sensor.html) (Optional, not needed for LED control)

## Setup
- Install [deCONZ](https://phoscon.de/en/conbee2/install) and [Rust](https://www.rust-lang.org/learn/get-started).
- Setup sensors and lights using Phoscon app. (Phoscon app url will be the api url too)
- Get public api key:
  - In phoscon app go to Gateway -> Advanced -> Authenticate app
  - using postamn send a POST request to http://<phoscon_url>:8080/api with the body "{"devicetype": "my application"}"
- Copy "config.example.toml" rename to "config.toml" and fill out the url and api key values.
- cargo run

## Goals
- Control LED light based on Light sensor value and time. Count the time when the plants gets enough Sunlight and fill the rest. Max time should come from config value.
- API to manually shut off automatation logic and get sensor values.
- Mobile / Web frontend app.
- Logging sensor values.

## Links
- [deCONZ REST-API](https://dresden-elektronik.github.io/deconz-rest-doc/)
