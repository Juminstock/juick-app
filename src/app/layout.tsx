import type { Metadata } from "next";
import { Inter } from "next/font/google";
import { Footer } from "app/components/shared/Footer";
import { Header } from "app/components/shared/Header";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Juick app",
  description: "Innovative payment system",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Header />
        {children}
        <Footer />
      </body>
    </html>
  );
}
