import { ChangeEventHandler, useState } from 'react';

export const useTextInput = (initValue = '') => {
  const [value, set] = useState(initValue);

  const onChange: ChangeEventHandler<HTMLInputElement | HTMLTextAreaElement> = (e) => {
    set(e.target.value);
  };

  const reset = () => set(initValue);

  return { value, set, onChange, reset };
};
