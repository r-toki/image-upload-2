export const encodeFile = (file: File): Promise<{ contentType: string; encoded: string }> =>
  new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => {
      const result = reader.result as string;
      const [contentType, encoded] = result.split(';base64,');
      resolve({ contentType, encoded });
    };
    reader.onerror = (error) => reject(error);
  });
