
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';

describe('Frontend Tests', () => {
  it('renders without crashing', () => {
    render(<div>Hello Test</div>);
    expect(screen.getByText('Hello Test')).toBeInTheDocument();
  });
});
