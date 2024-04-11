import { dirname, join } from 'path';
/** @type {import('next').NextConfig} */

const nextConfig = {
    sassOptions: {
        includePaths: [join(dirname('./'), 'src/sass')],
        prependData: `@import "main.sass"`,
    },
};

export default nextConfig;