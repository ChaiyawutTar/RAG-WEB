// frontend/utils/socket.ts
import { io, Socket } from 'socket.io-client';

let socket: Socket | null = null;

export const connectSocket = () => {
  if (!socket) {
    socket = io('http://localhost:8080');
  }
  return socket;
};

export const getSocket = () => {
  return socket;
};