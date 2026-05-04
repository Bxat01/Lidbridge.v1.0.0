import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'LidBridge',
  description: 'Clean local projects and push to GitHub — by Lidprex Labs',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
