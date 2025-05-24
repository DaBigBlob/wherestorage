// 500 to 65456             :::: 64957    states :: ~15 bits   :: 1.9 bytes
// 0 to 65536               :::: 65537    states :: ~16 bits   :: 2 bytes
// 1 to 9999999 (7 of them) :::: 9999999  states :: ~23 bits   :: 2.9 bytes
// 1 to 9999999 (7 of them) :::: 9999999  states :: ~23 bits   :: 2.9 bytes
// total log_2(9999999*9999999*65537*64957) = ~78 bits = ~9 bytes

we store the deltas in u16 = 2 bytes; net 9-2 = 7 bytes; 2:9 compression
the page_epoch is u64 = 8 bytes so storing the page_epoch is also not too bad; 8:9 compression

== out layout begin ==
<[encoding version]>
<[frame_epoch; u64; 8 bytes; 9 byte payload]>
[epoch_delta; u16; 2 bytes; 9 byte payload]
[epoch_delta; u16; 2 bytes; 9 byte payload]
.
. <(2^16 + 1) * 9 = 65537 * 9 = 589833 bytes; page_size = ~0.5mb; max>
.
[epoch_delta; u16; 2 bytes; 9 byte payload]
<[frame_epoch; u64; 8 bytes; 9 byte payload]>
.
.
.
== out layout end ==