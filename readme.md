## Oipc

### Funtional Requirements
1. 1:N communication shall be possible.
2. The sender and receivers share a piece of memory each. The owner has RW access but the counterpart(s) only have RO access.
3. The receiver(s) notify the sender when it is done reading from the memory.
4. Details like name, size, alignment of the shared memory is known to the sender as well as the receivers.

