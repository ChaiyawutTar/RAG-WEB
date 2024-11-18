// components/ChatList.tsx
'use client';

import { Button, ScrollShadow } from '@nextui-org/react';
import { FiPlus, FiMessageSquare, FiTrash2 } from 'react-icons/fi';
import { Chat } from '@/types/chat';
import { motion, AnimatePresence } from 'framer-motion';

interface ChatListProps {
  chats: Chat[];
  currentChatId: string | null;
  onSelectChat: (chatId: string) => void;
  onNewChat: () => void;
  onDeleteChat: (chatId: string) => void;
}

export default function ChatList({
  chats,
  currentChatId,
  onSelectChat,
  onNewChat,
  onDeleteChat
}: ChatListProps) {
  return (
    <div className="w-64 h-full flex flex-col border-r dark:border-gray-700">
      <div className="p-4">
        <Button
          color="primary"
          startContent={<FiPlus />}
          className="w-full"
          onClick={onNewChat}
        >
          New Chat
        </Button>
      </div>
      
      <ScrollShadow className="flex-1">
        <div className="space-y-1 p-2">
          <AnimatePresence>
            {chats.map((chat) => (
              <motion.div
                key={chat.id}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -20 }}
                transition={{ duration: 0.2 }}
              >
                <div className="relative group">
                  <Button
                    variant={currentChatId === chat.id ? "flat" : "light"}
                    className="w-full justify-start"
                    onClick={() => onSelectChat(chat.id)}
                  >
                    <div className="flex items-center gap-2 truncate">
                      <FiMessageSquare />
                      <span className="truncate">{chat.title}</span>
                    </div>
                  </Button>
                  <div className="absolute right-2 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity">
                    <Button
                      isIconOnly
                      size="sm"
                      variant="light"
                      onClick={(e) => {
                        e.stopPropagation();
                        onDeleteChat(chat.id);
                      }}
                      className="min-w-unit-8 w-unit-8 h-unit-8"
                    >
                      <FiTrash2 className="text-danger" />
                    </Button>
                  </div>
                </div>
              </motion.div>
            ))}
          </AnimatePresence>
        </div>
      </ScrollShadow>
    </div>
  );
}