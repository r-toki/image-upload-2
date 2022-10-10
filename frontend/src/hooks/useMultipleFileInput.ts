import { ChangeEventHandler, MouseEventHandler, useRef, useState } from 'react';

export const useMultipleFileInput = () => {
  const [value, set] = useState<File[]>([]);
  const ref = useRef<HTMLInputElement>(null);

  const onChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    const files = e.target.files;
    if (files) {
      const added: File[] = [];
      for (const file of files) {
        added.push(file);
      }
      set((prev) => [...prev, ...added]);
    }
    e.target.value = '';
  };

  const onClick: MouseEventHandler<HTMLButtonElement> = () => {
    ref.current?.click();
  };

  const remove = (index: number) => set((prev) => prev.filter((_, i) => index !== i));

  const reset = () => set([]);

  return {
    value,
    set,
    ref,
    onChange,
    onClick,
    remove,
    reset,
  };
};
