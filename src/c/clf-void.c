
// for gcc
// extern void __builtin___clear_cache (void *begin, void *end);
// for clang
// extern void __builtin___clear_cache (char *begin, char *end);

void _cache_line_flush(unsigned char *begin, unsigned char *end)
{
    // NOTHING TODO
}
