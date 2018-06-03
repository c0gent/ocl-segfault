__kernel void sequential_expand(
        unsigned long len,
        __global uchar const* const in,
        __global uchar* const out)
{
    uint const idx = get_global_id(0);
    uint j = 0;
    for (uint i = 0; i < len; ++i) {
        switch (in[i]) {
            case 'X':
                out[j + 0] = 'X';
                out[j + 1] = '+';
                out[j + 2] = 'Y';
                out[j + 3] = 'F';
                out[j + 4] = '+';
                j += 5;
                break;
            case 'Y':
                out[j + 0] = '-';
                out[j + 1] = 'F';
                out[j + 2] = 'X';
                out[j + 3] = '-';
                out[j + 4] = 'Y';
                j += 5;
                break;
            default:
                out[j++] = in[i];
                break;
        }
    }
}

__kernel void sequential_expand_n(
        unsigned long len,
        const unsigned long n,
        __global uchar* const in,
        __global uchar* const out)
{
    __global uchar* in_ = in;
    __global uchar* out_ = out;

    for (size_t i = 0; i < n; ++i) {
        sequential_expand(len, in_, out_);
        __global uchar* const temp = in_;
        in_ = out_;
        out_ = temp;
        len += 4*(1<<i);
    }

    if (n % 2 == 0) {
        for(unsigned long i = 0; in[i]; ++i) {
            out[i] = in[i];
        }
    }
}
