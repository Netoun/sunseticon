
.DEFAULT_GOAL := help
help: 
	@grep -E '(^[a-zA-Z_-]+:.*?##.*$$)|(^##)' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m##/[33m/'
.PHONY: help 
##
##
##             ^^                   @@@@@@@@@
##        ^^     |  ^^           @@@@@@@@@@@@@@@
##            \__|__           @@@@@@@@@@@@@@@@@@              ^^
##             \▒▒▒/          @@@@@@@@@@@@@@@@@@@@
##  ~~~~ ~~ ~~~~~ ~~~~~~~~ ~~ &&&&&&&&&&&&&&&&&&&& ~~~~~~~ ~~~~~~~~~~~ ~~~
##  ~         ~~   ~  ~       ~~~~~~~~~~~~~~~~~~~~ ~       ~~     ~~ ~
##    ~      ~~      ~~ ~~ ~~  ~~~~~~~~~~~~~ ~~~~  ~     ~~~    ~ ~~~  ~ ~~
##    ~  ~~     ~         ~      ~~~~~~  ~~ ~~~       ~~ ~ ~~  ~~ ~
##  ~  ~       ~ ~      ~           ~~ ~~~~~~  ~      ~~  ~             ~~
##        ~             ~        ~      ~      ~~   ~             ~
##--------------------------------------------------------------------------
##              ____ _  _ __ _ ____ ____ ____    __ ___ __  __ _ 
##             / ___/ )( (  ( / ___(  __(_  _)  (  / __/  \(  ( \
##             \___ ) \/ /    \___ \) _)  )(     )( (_(  O /    /
##             (____\____\_)__(____(____)(__)   (__\___\__/\_)__)
##
##--------------------------------------------------------------------------

install:
	cd vue && yarn
	cd back && cargo install