ifdef VERBOSE
	export RCORE_DEBUG_VERBOSE=1
endif

run: export RCORE_MODE=release
run:
	@cd os && make run

run-debug: export RCORE_MODE=debug
run-debug:
	@cd os && make run

debug:export RCORE_MODE=debug
debug:
	@cd os && make debug

gdbserver:export RCORE_MODE=debug
gdbserver:
	@cd os && make gdbserver

clean:
	@cd os && make clean
	@cd user && make clean