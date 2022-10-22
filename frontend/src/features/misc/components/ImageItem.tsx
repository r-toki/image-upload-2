export type ImageItemProps = {
  src: string;
};

export const ImageItem = ({ src }: ImageItemProps) => {
  return (
    <img
      src={src}
      width="200px"
      height="200px"
      style={{ border: '1px', borderRadius: '10px', objectFit: 'cover' }}
    />
  );
};
