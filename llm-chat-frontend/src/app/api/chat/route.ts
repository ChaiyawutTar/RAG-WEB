// frontend/app/api/chat/route.ts
import { NextResponse } from 'next/server'

export async function POST(req: Request) {
  const { prompt } = await req.json()
  
  const response = await fetch('http://localhost:8080/chat', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ prompt }),
  })

  const data = await response.json()
  return NextResponse.json(data)
}