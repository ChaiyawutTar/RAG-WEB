// components/ThemeSwitch.tsx
'use client';

import { useTheme } from 'next-themes';
import { Button } from '@nextui-org/react';
import { useEffect, useState } from 'react';
import { FiSun, FiMoon } from 'react-icons/fi';

export function ThemeSwitch() {
  const { theme, setTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return (
      <Button isIconOnly variant="light" disabled>
        <div className="w-5 h-5 animate-pulse bg-default-300 rounded-full" />
      </Button>
    );
  }

  return (
    <Button
      isIconOnly
      variant="light"
      onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
    >
      {theme === 'dark' ? <FiSun size={20} /> : <FiMoon size={20} />}
    </Button>
  );
}