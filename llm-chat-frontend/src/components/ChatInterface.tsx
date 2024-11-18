// components/ChatInterface.tsx
'use client';

import { useState, useEffect, useRef, Suspense } from 'react';
import { useSocket } from '@/contexts/SocketContext';
import { Input, Button, Card, Avatar } from '@nextui-org/react';
import { FiUser, FiSend, FiPlus } from 'react-icons/fi';
import { RiRobot2Line } from 'react-icons/ri';
import { ThemeSwitch } from './ThemeSwitch';
import { Chat, Message } from '@/types/chat';
import { v4 as uuidv4 } from 'uuid';
import dynamic from 'next/dynamic';

const ChatList = dynamic(() => import('./ChatList'), {
  ssr: false,
});

const LoadingState = () => (
  <div className="w-64 h-full flex flex-col border-r dark:border-gray-700 animate-pulse">
    <div className="p-4">
      <div className="h-10 bg-default-200 rounded-lg" />
    </div>
    <div className="flex-1 p-2 space-y-2">
      {[1, 2, 3].map((i) => (
        <div key={i} className="h-12 bg-default-200 rounded-lg" />
      ))}
    </div>
  </div>
);

export default function ChatInterface() {
  const [mounted, setMounted] = useState(false);
  const { socket, isConnected } = useSocket();
  const [prompt, setPrompt] = useState('');
  const [chats, setChats] = useState<Chat[]>([]);
  const [currentChatId, setCurrentChatId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const savedChats = localStorage.getItem('chats');
    if (savedChats) {
      setChats(JSON.parse(savedChats));
    }
    setMounted(true);
  }, []);

  useEffect(() => {
    if (chats.length > 0) {
      localStorage.setItem('chats', JSON.stringify(chats));
    }
  }, [chats]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    const currentChat = chats.find(chat => chat.id === currentChatId);
    if (currentChat?.messages) {
      scrollToBottom();
    }
  }, [chats, currentChatId]);

  useEffect(() => {
    if (!socket) return;

    socket.on('result', (result: string) => {
      if (!currentChatId) return;
      
      setChats(prev => prev.map(chat => {
        if (chat.id === currentChatId) {
          return {
            ...chat,
            messages: [...chat.messages, {
              type: 'result' as 'result',
              content: result,
              timestamp: new Date()
            }],
            updatedAt: new Date()
          };
        }
        return chat;
      }));
      setIsLoading(false);
    });

    return () => {
      socket.off('result');
    };
  }, [socket, currentChatId]);

  const createNewChat = () => {
    const newChat: Chat = {
      id: uuidv4(),
      title: `New Chat ${chats.length + 1}`,
      messages: [],
      createdAt: new Date(),
      updatedAt: new Date()
    };
    setChats(prev => [...prev, newChat]);
    setCurrentChatId(newChat.id);
  };

  const deleteChat = (chatId: string) => {
    setChats(prev => prev.filter(chat => chat.id !== chatId));
    if (currentChatId === chatId) {
      setCurrentChatId(null);
    }
  };

  const sendPrompt = () => {
    if (!socket || !prompt.trim() || isLoading || !currentChatId) return;

    setIsLoading(true);
    socket.emit('prompt', prompt);

    setChats(prev => prev.map(chat => {
      if (chat.id === currentChatId) {
        const updatedMessages = [...chat.messages, {
          type: 'prompt' as 'prompt',
          content: prompt,
          timestamp: new Date()
        }];
        
        const title = chat.messages.length === 0 
          ? prompt.slice(0, 30) + (prompt.length > 30 ? '...' : '')
          : chat.title;

        return {
          ...chat,
          title,
          messages: updatedMessages,
          updatedAt: new Date()
        };
      }
      return chat;
    }));
    
    setPrompt('');
  };

  const currentChat = chats.find(chat => chat.id === currentChatId);

  if (!mounted) {
    return (
      <div className="h-screen flex">
        <LoadingState />
        <div className="flex-1 flex items-center justify-center">
          <div className="animate-pulse">
            <div className="h-8 w-48 bg-default-200 rounded-lg mb-4" />
            <div className="h-10 w-32 bg-default-200 rounded-lg mx-auto" />
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen flex">
      <Suspense fallback={<LoadingState />}>
        <ChatList
          chats={chats}
          currentChatId={currentChatId}
          onSelectChat={setCurrentChatId}
          onNewChat={createNewChat}
          onDeleteChat={deleteChat}
        />
      </Suspense>

      <div className="flex-1">
        {currentChat ? (
          <Card className="w-full h-full rounded-none">
            <div className="flex justify-between items-center p-4 border-b">
              <div className="flex items-center gap-2">
                <RiRobot2Line className="w-6 h-6" />
                <h3 className="text-xl font-bold truncate">{currentChat.title}</h3>
                <span className={`text-sm ${isConnected ? 'text-success' : 'text-danger'}`}>
                  {isConnected ? "Connected" : "Disconnected"}
                </span>
              </div>
              <ThemeSwitch />
            </div>
            
            <div className="flex-1 overflow-y-auto p-4 space-y-4">
              {currentChat.messages.map((message, index) => (
                <div
                  key={index}
                  className={`flex ${message.type === 'prompt' ? 'justify-end' : 'justify-start'} gap-2`}
                >
                  {message.type === 'result' && (
                    <Avatar
                      icon={<RiRobot2Line size={20} />}
                      className="w-8 h-8 bg-primary/10"
                    />
                  )}
                  
                  <div className={`max-w-[70%] ${
                    message.type === 'prompt' ? 'bg-primary/10' : 'bg-default-100'
                  } rounded-lg p-3`}>
                    <p className="whitespace-pre-wrap">{message.content}</p>
                    <p className="text-right text-tiny text-default-400">
                      {new Date(message.timestamp).toLocaleTimeString()}
                    </p>
                  </div>

                  {message.type === 'prompt' && (
                    <Avatar
                      icon={<FiUser size={20} />}
                      className="w-8 h-8 bg-primary/20"
                    />
                  )}
                </div>
              ))}
              <div ref={messagesEndRef} />
            </div>

            <div className="p-4 border-t">
              <div className="flex gap-2">
                <Input
                  className="flex-1"
                  placeholder="Type your message..."
                  value={prompt}
                  onChange={(e) => setPrompt(e.target.value)}
                  onKeyPress={(e) => e.key === 'Enter' && !e.shiftKey && sendPrompt()}
                  disabled={isLoading}
                  endContent={
                    <Button
                      isIconOnly
                      color="primary"
                      size="sm"
                      variant="light"
                      onClick={sendPrompt}
                      isLoading={isLoading}
                      disabled={!prompt.trim()}
                    >
                      {!isLoading && <FiSend size={20} />}
                    </Button>
                  }
                />
              </div>
            </div>
          </Card>
        ) : (
          <div className="h-full flex items-center justify-center">
            <div className="text-center">
              <h2 className="text-2xl font-bold mb-4">Welcome to AI Chat</h2>
              <Button
                color="primary"
                startContent={<FiPlus />}
                onClick={createNewChat}
              >
                Start a new chat
              </Button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}