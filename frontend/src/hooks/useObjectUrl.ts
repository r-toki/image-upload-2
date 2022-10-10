import { useEffect, useState } from 'react';

export const useObjectUrl = (object?: File | Blob) => {
  const [objectUrl, setObjectUrl] = useState<string>();

  useEffect(() => {
    if (!object) setObjectUrl(undefined);
    else setObjectUrl(URL.createObjectURL(object));
    return () => {
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [object]);

  return { objectUrl };
};
