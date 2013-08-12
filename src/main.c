int main(struct multiboot *mboot_ptr){
	fb_clear();
	fb_write_str("Hello, world!\n");
	return 0xDEADBEEF;
}