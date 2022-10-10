import { ChangeEventHandler, MouseEventHandler, useRef, useState } from 'react';

export const useFileInput = () => {
  const [value, set] = useState<File>();

  const ref = useRef<HTMLInputElement>(null);

  const onChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    const files = e.target.files;
    if (files && files[0]) set(files[0]);
    e.target.value = '';
  };

  const onClick: MouseEventHandler<HTMLButtonElement> = () => {
    ref.current?.click();
  };

  const reset = () => set(undefined);

  return {
    value,
    set,
    ref,
    onChange,
    onClick,
    reset,
  };
};
