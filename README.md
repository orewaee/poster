![](/docs/assets/banner.png)

## Commands

* `poster init [path]` - init the config using the specific path
* `poster run [--host <HOST>] [--port <PORT>] [--static-path <STATIC_PATH>]` - run the poster instance

## TODO

* [ ] security
    * [ ] tls listener (via flag)
    * [ ] static access by request (temporary link to static resource? like session but after auth and for every request random link)
* [ ] global refactoring
    * [ ] app module
        * [ ] http router
        * [ ] http handlers
* [ ] cli
    * [ ] database location flag
    * [ ] init database
    * [ ] verbose flag for all commands
* [ ] logs
* [ ] collect statistics for analisis
* [ ] tui?
    * [ ] statistics and analisis
    * [ ] logs
    * [ ]
* [ ] docs
