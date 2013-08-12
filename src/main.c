int main(struct multiboot *mboot_ptr){
	fb_clear();
	fb_write_str("Hello, world!\n\n");
	int i;
	for( i=0; i<24; i++ ){
		fb_set_colors(i%16, 0);
		fb_write_int( i%16 );
		fb_write_char('\n');
	}
	return 0xDEADBEEF;
}