# BotfloxAlltalks
lightweight ffxiv dicord bot

## Running
To run Botflox Alltalks using docker, first build the image with

`docker build -t botflox .`

And then run the built image with

`docker run -d botflox`

Or, run `./run-docker.sh --build` for first run and `run-docker.sh` for subsequent runs.

You can also run Botflox using docker from the docker hub without cloning the repository!
Just run

`docker run -d robotbrain/botflox-alltalks`
