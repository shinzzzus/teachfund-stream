import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "TeachFund Stream",
  description: "Milestone-based teacher training crowdfunding where Stellar escrow unlocks grants after verified learning outcomes.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
