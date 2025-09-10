#include "core_bridge/src/lib.rs.h"
#include "core_bridge/snes9x/snes9x.h"


void S9xMessage (int type, int number, const char *message){
	rust_s9x_message(type, number, message);
}

void S9xExtraUsage() {
	return;
}
