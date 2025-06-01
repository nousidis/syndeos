import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { DayPicker } from 'react-day-picker';
import * as dateFns from 'date-fns';

describe('react-day-picker and date-fns compatibility', () => {
  it('DayPicker renders without crashing', () => {
    render(<DayPicker />);
  });

  it('DayPicker formats dates correctly with date-fns', () => {
    const today = new Date();

    render(
      <DayPicker
        month={today}
        formatters={{
          formatMonthCaption: (date) => dateFns.format(date, 'MMMM yyyy'),
          formatWeekdayName: (date) => dateFns.format(date, 'EEE'),
        }}
      />
    );

    const monthCaption = dateFns.format(today, 'MMMM yyyy');
    expect(screen.getByText(monthCaption)).toBeInTheDocument();
  });

  it('renders with correct month and allows interactions', async () => {
    const onSelectSpy = vi.fn();
    const today = new Date();
    const currentMonth = dateFns.format(today, 'MMMM yyyy');
    
    const { container } = render(
      <DayPicker
        mode="single"
        selected={today}
        onSelect={onSelectSpy}
        defaultMonth={today}
      />
    );

    expect(screen.getByText(currentMonth)).toBeInTheDocument();
    
    const dayCells = container.querySelectorAll('.rdp-day');
    expect(dayCells.length).toBeGreaterThan(0);
    
    const dayToClick = Array.from(dayCells).find(cell =>
      cell.getAttribute('aria-disabled') !== 'true' && 
      !cell.classList.contains('rdp-day_outside')
    );
    
    if (dayToClick) {
      await userEvent.click(dayToClick);
      expect(onSelectSpy).toHaveBeenCalled();
    } else {
      throw new Error("No clickable day found in calendar");
    }
  });

  it('DayPicker uses complex date-fns features correctly', () => {
    const startDate = dateFns.startOfWeek(new Date());
    const endDate = dateFns.endOfWeek(new Date());

    render(
      <DayPicker
        mode="range"
        defaultMonth={startDate}
        selected={{
          from: startDate,
          to: endDate
        }}
      />
    );
  });
});