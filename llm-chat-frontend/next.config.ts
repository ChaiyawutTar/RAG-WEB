/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: '/socket.io/:path*',
        destination: 'http://localhost:8080/socket.io/:path*',
      },
    ];
  },
  // Enable if you need to disable strict mode for socket.io
  // reactStrictMode: false,
  reactStrictMode: true,
  swcMinify: true,
}

module.exports = nextConfig