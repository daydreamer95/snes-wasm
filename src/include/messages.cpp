#include "snes-wasm/src/main.rs.h"
#include "snes-wasm/snes9x/snes9x.h"


void S9xMessage (int type, int number, const char *message){
	rust_s9x_message(type, number, message);
}

void S9xExtraUsage() {
	return;
}
