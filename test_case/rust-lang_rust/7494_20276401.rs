
% cat Makefile
default: printMSG

MSG="Hello World"
WHO="someone"

ifeq "$(origin WHO)" "environment"
  WHO_FROM_ENV = $(WHO)
endif

ifeq "$(origin WHO)" "environment override"
  WHO_FROM_ENV_OVERRIDE = $(WHO)
  MSG += $(WHO)
endif

ifeq "$(origin WHO)" "command line"
  WHO_FROM_CMD = $(WHO)
  MSG += $(WHO)
endif

printMSG:
    @echo $(MSG)
% make
Hello World
% make WHO=you
Hello World you
% WHO=you make
Hello World
% WHO=forced-via-dash-e make -e
Hello World forced-via-dash-e
% 
